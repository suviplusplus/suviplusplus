<script lang="ts">
    import BlogComment from "$lib/components/BlogComment.svelte";
    import { env } from "$env/dynamic/public";
    import { writable } from "svelte/store";
    import { onMount } from "svelte";
    import { browser } from '$app/environment';
    import { page } from '$app/state';
	import { marked } from "marked";
    import DOMPurify from "dompurify";


    const apiUrl = browser ? env.PUBLIC_API_URL_BROWSER : env.PUBLIC_API_URL;

    interface Comment {
        id: string,
        author: string;
        body: string,
        date: Date
    }

    interface Post {
        id: string,
        title: string;
        body: string;
        date: Date;
        comments: Comment[];
    }

    let post = writable<Post>({id: page.params.id, title: "", body: "", date: new Date()} as Post);

    async function getPost(): Promise<Post> {
        return fetch(apiUrl + '/api/blog/'+page.params.id)
            .then(res => res.json())
            .then(res => {
                const p = res as Post;
                return {
                    ...p,
                    date: new Date(p.date),
                    comments: (p.comments as Comment[]).map(c => ({
                        ...c,
                        date: new Date(c.date)
                    }))
                };
            });
    }

    onMount(async () => {
        const fetched = await getPost();
        fetched.comments.sort((a, b) => {
            return a.date.getTime() - b.date.getTime();
        });

        post.set(fetched);
    });
</script>

<div>
    <h2>{$post.title}</h2>

    <!-- eslint-disable-next-line svelte/no-at-html-tags -->
    {@html browser ? DOMPurify.sanitize(marked.parse($post.body) as string) : marked.parse($post.body) as string}

    <small>{$post.date.toLocaleString('en-US', {
        year: 'numeric',
        month: 'short',
        day: 'numeric',
        hour: "2-digit",
        minute: "2-digit",
        hour12: false
    }).toLowerCase()}</small>

    {#each $post.comments as { id, author, body, date } (id)}
        <BlogComment author={author} body={body} date={date} />
    {/each}
</div>
