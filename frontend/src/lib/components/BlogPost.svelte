<script lang="ts">
    import { env } from '$env/dynamic/public'; 
	import BlogComment from "./BlogComment.svelte";
    import { marked } from 'marked';
    import DOMPurify from "dompurify";
    import { browser } from '$app/environment';

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

    let commentBody = $state("wow suvi, you're so cool and pretty!")
    let commentAuthor = $state("")

    async function postComment(id: string) {
        let newComment: NewComment = { author: commentAuthor, body: commentBody }
        const response = await fetch(apiUrl+"/api/blog/"+id+"/comments", {
            method: "POST",
            body: JSON.stringify(newComment),
            headers: {
				'Content-Type': 'application/json'
			}
        });

        await onCommentAdded();

        commentBoxOpen = false;

        return response
    }

    let commentBoxOpen = $state(false);

    let { id, title, body, date, comments, onCommentAdded }: Props = $props();

</script>

<style>

    @import '../../styles.css';

</style>

<div class="blogpost">
    <h2>{title}</h2>
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
        comment on this!
    </button>
    {#if commentBoxOpen}
        <div>
            <textarea rows="3" bind:value={commentBody}></textarea><br />
            <small>your nickname:</small><input type="text" style="width:auto;" bind:value={commentAuthor} /><br />
            <button type="button" onclick={() => postComment(id)}>comment!</button>
        </div>
    {/if}
    {#each comments as { id, author, body, date } (id)} 
        <BlogComment author={author} body={body} date={date} />
    {/each}
</div>