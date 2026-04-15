<script lang="ts">
	import { goto } from "$app/navigation";

    let {dest}: {dest: string} = $props();
    function onClick(): void {
        // eslint-disable-next-line svelte/no-navigation-without-resolve
        goto(dest);
    }
</script>

<button class="navbutton" onclick={() => onClick()}><span style="color:purple">goto</span> <span style="color:darkgoldenrod">{dest}</span><span style="color:darkslategray">;</span></button>