<script lang="ts">
	import { env } from '$env/dynamic/public';
    import { onMount } from "svelte";
	import { writable } from 'svelte/store';
    import { fade } from 'svelte/transition';
    import { browser } from '$app/environment';
    const count = writable(0);
    const apiUrl = browser ? env.PUBLIC_API_URL_BROWSER : env.PUBLIC_API_URL;

    interface Suvi {
        value: number;
    }

    interface UpdateResponse {
        value: number;
    }

    function updateSuvi(): Promise<Suvi> {
        return fetch(apiUrl + "/api/suvi")
        .then(res => res.json())
        .then(res => {
            const suvi = res as Suvi;
            count.set(suvi.value);
            return suvi;
        });
    }

    function incrementSuvi(): Promise<number> {
        return fetch(apiUrl + "/api/suvi", { method: 'POST' })
        .then(res => res.json())
        .then(res => {
            return (res as UpdateResponse).value
        })
    }

    async function buttonHandler() {
        const updated = await incrementSuvi();
        count.set(updated)
    }

    onMount(() => {
        updateSuvi();

        const interval = setInterval(updateSuvi, 10000);
        return () => clearInterval(interval);
    })
    
</script>

<style>
    @import "../../styles.css"
</style>

<div class="titlebox">
    <p class="suvi"><span style="color:olivedrab">// suvi.cc</span><br />
    <span style="color:royalblue">int</span> <span style="color:darkgoldenrod">suvi</span> <span style="color:darkslategray">=</span> {#key $count}<span style="color:darkcyan" in:fade>{$count}</span>{/key}<span style="color:darkslategray">;</span></p>
    <button class="suvi button" on:click={()=>buttonHandler()}><span style="color:darkgoldenrod">suvi</span><span style="color:darkslategray">++;</span></button>
</div>