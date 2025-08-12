<script lang="ts">
<<<<<<< HEAD
=======
	import Counter from './Counter.svelte';
	import welcome from '$lib/images/svelte-welcome.webp';
	import welcomeFallback from '$lib/images/svelte-welcome.png';
>>>>>>> origin/master
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
<<<<<<< HEAD
	<link rel="icon" href="data:,">
</svelte:head>

<section class="banner-section">
	<img src="/src/lib/images/banner -image.jpg" alt="Banner" class="hero-banner" />
</section>

<main>
	<section class="new-in-section">
		<h2>New In</h2>
		<p>Check out the latest arrivals and exclusive drops.</p>
		<button class="discover-btn">Discover More</button>
	</section>
</main>

<style>
	.banner-section {
		padding: 0;
		margin: 0;
		width: 100%;
	}

	.hero-banner {
		width: 100%;
		height: 90vh;
		object-fit: cover;
		display: block;
		margin: 0;
		padding: 0;
		border: none;
	}

	.new-in-section {
		width: 100%;
		max-width: 1200px;
		margin: 0 auto;
		padding: 3rem 1rem 2rem 1rem;
		display: flex;
		flex-direction: column;
		align-items: center;
		background: var(--primary-bg, #fff);
		box-sizing: border-box;
	}

	.new-in-section h2 {
		font-size: 2rem;
		margin-bottom: 0.5rem;
		font-weight: 700;
		letter-spacing: 1px;
	}

	.new-in-section p {
		font-size: 1.1rem;
		margin-bottom: 1.5rem;
		color: var(--secondary-text, #555);
	}

	.discover-btn {
		padding: 0.75rem 2rem;
		font-size: 1rem;
		font-weight: 600;
		background: var(--primary-hover, #3b82f6);
		color: #fff;
		border: none;
		border-radius: 0.375rem;
		cursor: pointer;
		transition: background 0.2s;
	}

	.discover-btn:hover {
		background: var(--primary-color, #1f2937);
=======
</svelte:head>

<section class="banner-section">
	<img src="/src/lib/images/banner -image.jpg" alt="Banner" class="hero-banner" />
</section>

<main>
	<section class="new-in-section">
		<h2>New In</h2>
		<p>Check out the latest arrivals and exclusive drops.</p>
		<button class="discover-btn">Discover More</button>
	</section>
</main>

<style>
	.banner-section {
		padding: 0;
		margin: 0;
		width: 100%;
	}

	.hero-banner {
		width: 100%;
		height: 90vh;
		object-fit: cover;
		display: block;
		margin: 0;
		padding: 0;
		border: none;
	}

	.new-in-section {
		width: 100%;
		max-width: 1200px;
		margin: 0 auto;
		padding: 3rem 1rem 2rem 1rem;
		display: flex;
		flex-direction: column;
		align-items: center;
		background: var(--primary-bg, #fff);
		box-sizing: border-box;
	}

	.new-in-section h2 {
		font-size: 2rem;
		margin-bottom: 0.5rem;
		font-weight: 700;
		letter-spacing: 1px;
	}

	.new-in-section p {
		font-size: 1.1rem;
		margin-bottom: 1.5rem;
		color: var(--secondary-text, #555);
	}

	.welcome img {
		position: absolute;
		width: 100%;
		height: 100%;
		top: 0;
		display: block;
>>>>>>> origin/master
	}
</style>
