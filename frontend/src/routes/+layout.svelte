<script lang="ts">
	import favicon from '$lib/assets/favicon.svg';
    import Suvi from '$lib/components/Suvi.svelte';
	import NavButton from '$lib/components/NavButton.svelte';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<style>
	@import '../styles.css';
</style>

<Suvi />

<nav>
	<NavButton dest="/" />
	-
	<NavButton dest="/blog" />
</nav>
{@render children?.()}
