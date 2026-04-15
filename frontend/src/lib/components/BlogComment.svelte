<script lang="ts">
    interface Comment {
        author: string;
        body: string,
        date: Date
    }
    let { body, author, date }: Comment = $props();
</script>

<style>

    @import '../../styles.css';

</style>

<div class="comment">
    <h3>{author} said:</h3>
    <p>{body}</p>
    <small>{date.toLocaleString('en-US', {
        year: 'numeric',
        month: 'short',
        day: 'numeric',
        hour: "2-digit",
        minute: "2-digit",
        hour12: false
    }).toLowerCase()}</small>
</div>