<script lang="ts">
    import BlogPost from "$lib/components/BlogPost.svelte";
    import { env } from "$env/dynamic/public";
    import { writable } from "svelte/store";
    import { onMount } from "svelte";
    import { browser } from '$app/environment';

    const apiUrl = browser ? env.PUBLIC_API_URL_BROWSER : env.PUBLIC_API_URL;

    interface Post {
        id: string;
        title: string;
        body: string;
        date: Date;
        comments: Comment[];
    }
    interface Comment {
        id: string,
        author: string;
        body: string,
        date: Date
    }

    let posts = writable<Post[]>([]);

    async function getPosts(): Promise<Post[]> {
        return fetch(apiUrl + '/api/blog')
            .then(res => res.json())
            .then(res => {
                return (res as Post[]).map(p => ({
                    ...p,
                    date: new Date(p.date),
                    comments: (p.comments as Comment[]).map(c => ({
                        ...c,
                        date: new Date(c.date)
                    }))
                }));
            });
    }

    async function updateComments(id: string): Promise<void> {
        const newComments: Comment[] = await fetch(apiUrl + '/api/blog/' + id + '/comments')
            .then(res => res.json())
            .then(res => {
                return (res as Comment[]).map(c => ({
                    ...c,
                    date: new Date(c.date),
                }));
            });

            posts.update(current => 
                current.map(post => 
                    post.id === id
                        ? { ...post, comments: newComments }
                        : post
                )
            )
    }

    onMount(async () => {
        const fetched = await getPosts();
        fetched.sort((a, b) => {
            return b.date.getTime() - a.date.getTime();
        });
        fetched.forEach((post) => {
            post.comments.sort((a, b) => {
                return a.date.getTime() - b.date.getTime();
            })
        })

        posts.set(fetched);
    })
</script>

<div>
    {#each $posts as post (post.id)}
        <BlogPost id={post.id} title={post.title} body={post.body} date={post.date} comments={post.comments} onCommentAdded={() => updateComments(post.id)} />
    {/each}
</div>
