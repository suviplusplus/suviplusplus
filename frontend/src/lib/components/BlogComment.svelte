<script lang="ts">
    interface Comment {
        author: string;
        body: string,
        date: Date
    }
    let { body, author, date }: Comment = $props();
</script>

<div class="comment">
    <span style="font-weight:300; color:darkslategray;">{author} said:</span><br />
    {body}<br />
    <small>{date.toLocaleString('en-US', {
        year: 'numeric',
        month: 'short',
        day: 'numeric',
        hour: "2-digit",
        minute: "2-digit",
        hour12: false
    }).toLowerCase()}</small>
</div>