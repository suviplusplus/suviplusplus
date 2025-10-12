<script lang="ts">
	import { env } from '$env/dynamic/public';
    import { onMount, tick } from "svelte";
	import { writable } from 'svelte/store';
    import { fade } from 'svelte/transition';
    const count = writable(0);
    const apiUrl = env.PUBLIC_API_URL;

    interface Suvi {
        value: number;
    }

    interface UpdateResponse {
        success: boolean;
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

    function incrementSuvi(): Promise<boolean> {
        return fetch(apiUrl + "/api/suvi", { method: 'POST' })
        .then(res => res.json())
        .then(res => {
            return (res as UpdateResponse).success
        })
    }

    async function buttonHandler() {
        await incrementSuvi();
        await updateSuvi();
    }

    onMount(async () => {
        await updateSuvi();
    })

    setInterval(updateSuvi, 10000);
    
</script>

<style>
    .titlebox {
        padding: 20px;
        width: 25em;
    }
    .suvi {
        font-family: monospace; 
        font-size: 3vh;
        color: darkslategray;
        margin: 0px;
    }
    .button {
        background-color: white; 
        border-radius: 1vh; 
        border: 3px solid darkslategray;
        margin: 0px;
    }
    .button:hover {
        background-color:darkcyan;
        transition-duration: 0.25s;
    }
    .button:active {
        background-color: cadetblue;
        transition-duration: 0ms;
    }

</style>

<div class="titlebox">
    <p class="suvi"><span style="color:olivedrab">// suviplusplus.fi</span><br /><span style="color:royalblue">int</span> <span style="color:darkgoldenrod">suvi</span> <span style="color:darkslategray">=</span> {#key $count}<span style="color:darkcyan" in:fade>{$count}</span>{/key}<span style="color:darkslategray">;</span></p>
    <button class="suvi button" on:click={()=>buttonHandler()}><span style="color:darkgoldenrod">suvi</span><span style="color:darkslategray">++;</span></button>
    <p></p>
</div>