<script lang="ts">
	import { goto } from "$app/navigation";

    let {dest}: {dest: string} = $props();
    function onClick(): void {
        // eslint-disable-next-line svelte/no-navigation-without-resolve
        goto(dest);
    }
</script>

<button class="navbutton" onclick={() => onClick()}>goto {dest};</button>