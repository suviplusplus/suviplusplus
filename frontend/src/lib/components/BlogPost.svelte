<script lang="ts">
    import { env } from '$env/dynamic/public'; 
	import BlogComment from "./BlogComment.svelte";
    import { marked } from 'marked';
    import DOMPurify from "dompurify";
    import { browser } from '$app/environment';
	import { onMount } from 'svelte';

    const apiUrl = browser ? env.PUBLIC_API_URL_BROWSER : env.PUBLIC_API_URL;

    interface Comment {
        id: string,
        author: string;
        body: string,
        date: Date
    }
    interface NewComment {
        author: string;
        body: string;
    }
    interface Props  {
        id: string,
        title: string;
        body: string;
        date: Date;
        comments: Comment[];
        onCommentAdded: () => Promise<void>;
    }

    interface CommentStatus {
        status: string,
        error: boolean
    }

    let commentsTruncated = $state(false);

    let comment = $state({ author: "", body: "" } as NewComment)
    let commentStatus = $state({status: "", error: false} as CommentStatus);

    async function postComment(id: string) {
        const response = await fetch(apiUrl+"/api/blog/"+id+"/comments", {
            method: "POST",
            body: JSON.stringify(comment),
            headers: {
				'Content-Type': 'application/json'
			}
        });

        if (response.status === 400) {
            commentStatus.error = true;
            commentStatus.status = "error sending comment";
        } else {
            commentStatus.error = false;
            commentStatus.status = "comment sent!";
        }

        await onCommentAdded();

        commentBoxOpen = false;
        comment.body = "";
        comment.author = "";

        return response
    }

    onMount(() => {
        if (comments.length > 3) {
            commentsTruncated = true;
        }
        comments = comments.slice(0, 3);
    });

    let commentBoxOpen = $state(false);

    let { id, title, body, date, comments, onCommentAdded }: Props = $props();

</script>

<div class="blogpost">
    <!-- eslint-disable-next-line svelte/no-navigation-without-resolve -->
    <h2><a href="/blog/{id}">{title}</a></h2>
    <!-- eslint-disable-next-line svelte/no-at-html-tags -->
    {@html DOMPurify.sanitize(marked.parse(body) as string)}
    <small>{date.toLocaleString('en-US', {
        year: 'numeric',
        month: 'short',
        day: 'numeric',
        hour: "2-digit",
        minute: "2-digit",
        hour12: false
    }).toLowerCase()}</small>
    <button type="button" onclick={() => commentBoxOpen = !commentBoxOpen}>
        {#if commentBoxOpen}
        close comment box
        {:else}
        comment on this!
        {/if}
    </button>
    {#if !(commentStatus.status === "") }
        <span style="color:{commentStatus.error ? "red" : "green"}">{commentStatus.status}</span>
    {/if}
    {#if commentBoxOpen}
        <div class="commentbox">
            <textarea spellcheck=false rows="3" placeholder="wow suvi, you're so cool and pretty!" bind:value={comment.body}></textarea><br />
            <small>your nickname:</small><input type="text" placeholder="anonymous" style="width:auto;" bind:value={comment.author} /><br />
            <button type="button" onclick={() => postComment(id)}>comment!</button>
        </div>
    {/if}
    {#each comments as { id, author, body, date } (id)} 
        <BlogComment author={author} body={body} date={date} />
    {/each}
    {#if commentsTruncated}
        <small>see more comments on the post page!</small>
    {/if}
</div>