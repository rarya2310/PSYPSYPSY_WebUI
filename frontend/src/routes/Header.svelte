<script lang="ts">
<<<<<<< HEAD
	import { page } from '$app/stores';
	import SlidingBanner from '$lib/elements/SlidingBanner.svelte';
	import BaseNavLink from '$lib/elements/BaseNavLink.svelte';
	import MobileNavBar from '$lib/elements/MobileNavBar.svelte';
	import { onMount } from 'svelte';

	let theme: 'light' | 'dark' = 'light';
	let mobileMenuOpen = false;
	let logoSize = '2.5rem';

	function applyTheme(newTheme: 'light' | 'dark') {
		// Remove any existing theme attributes
		document.documentElement.removeAttribute('data-theme');
		document.body.removeAttribute('data-theme');
		document.documentElement.classList.remove('dark');
		
		// Apply the new theme
		if (newTheme === 'dark') {
			document.documentElement.setAttribute('data-theme', 'dark');
			document.body.setAttribute('data-theme', 'dark');
			document.documentElement.classList.add('dark');
		} else {
			document.documentElement.setAttribute('data-theme', 'light');
			document.body.setAttribute('data-theme', 'light');
		}
		
		theme = newTheme;
		localStorage.setItem('theme', newTheme);
	}

	onMount(() => {
		// Check for saved theme preference or default to system preference
		const stored = localStorage.getItem('theme') as 'light' | 'dark' | null;
		
		if (stored) {
			applyTheme(stored);
		} else {
			// Check system preference
			const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
			applyTheme(prefersDark ? 'dark' : 'light');
		}
		
		const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
		const handleChange = (e: MediaQueryListEvent) => {
			if (!localStorage.getItem('theme')) {
				applyTheme(e.matches ? 'dark' : 'light');
			}
		};
		mediaQuery.addEventListener('change', handleChange);
        
		// Scroll handler: toggle logo size
		const handleScroll = () => {
			if (window.scrollY > 10) {
				logoSize = '2rem';
			} else {
				logoSize = '2.5rem';
			}
		};
		window.addEventListener('scroll', handleScroll);
        
		// Cleanup
		return () => {
			mediaQuery.removeEventListener('change', handleChange);
			window.removeEventListener('scroll', handleScroll);
		};
	});

	function toggleTheme() {
		const newTheme = theme === 'dark' ? 'light' : 'dark';
		applyTheme(newTheme);
	}
	
	function toggleMobileMenu() {
		mobileMenuOpen = !mobileMenuOpen;
	}
	
	function handleMenuClick(event: MouseEvent) {
		event.stopPropagation();
		mobileMenuOpen = !mobileMenuOpen;
		console.log('Mobile menu toggled:', mobileMenuOpen);
	}

</script>

<header>
	<SlidingBanner messages={[
		'PSYPSYPSY',
		'It\'s a Vice.',
		'Shipping all over India.',
		'Hypnosis in thread.'
	]} interval={2500} />
	
	<nav class="main-nav">
		<div class="nav-left">
			<!-- Mobile menu button -->
			<button class="mobile-menu-toggle" on:click={handleMenuClick} aria-label="Toggle navigation menu">
				MENU
			</button>
			
			<!-- Desktop nav links -->
			<div class="desktop-nav-links">
				<BaseNavLink href="/apparels" active={$page.url.pathname.toString() === '/apparels'} variant="primary">APPARELS</BaseNavLink>
				<BaseNavLink href="/collections" active={$page.url.pathname.toString() === '/collections'} variant="primary">COLLECTIONS</BaseNavLink>
				<BaseNavLink href="/new-in" active={$page.url.pathname.toString() === '/new-in'} variant="primary">NEW IN</BaseNavLink>
			</div>
		</div>
		
		<div class="nav-center">
			<a href="/" class="logo" style="--logo-size: {logoSize}">PSYPSYPSY</a>
		</div>
		
	   <div class="nav-right">
		 <button class="theme-toggle" on:click={toggleTheme} aria-label="Toggle dark/light mode">
		   {#if theme === 'dark'}
			 <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12.79A9 9 0 1 1 11.21 3a7 7 0 0 0 9.79 9.79z"></path></svg>
		   {:else}
			 <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="5"></circle><path d="M12 1v2m0 18v2m11-11h-2M3 12H1m16.95 6.95-1.41-1.41M6.34 6.34 4.93 4.93m12.02 0-1.41 1.41M6.34 17.66l-1.41 1.41"/></svg>
		   {/if}
		 </button>
	   </div>
	   
	   <!-- Mobile Navigation Dropdown -->
	   <MobileNavBar bind:isOpen={mobileMenuOpen} />
	</nav>
</header>

<style>
header {
	display: flex;
	flex-direction: column;
	position: sticky;
	top: 0;
	z-index: 100;
	background: var(--primary-bg);
}

	.main-nav {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.5rem 2rem;
		background: var(--primary-bg);
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
		position: relative;
	}	.nav-left,
	.nav-right {
		display: flex;
		gap: 2rem;
		flex: 1;
	}

	.nav-right {
		justify-content: flex-end;
	}

	.nav-center {
		position: absolute;
		left: 50%;
		transform: translateX(-50%);
		z-index: 10;
	}

	.logo {
		font-family: 'Times New Roman', serif;
		font-size: var(--logo-size, 3rem);
		font-weight: bold;
		color: var(--primary-color);
		text-decoration: none;
		letter-spacing: 0.05em;
		transition: color 0.3s ease, font-size 0.2s ease;
		text-transform: uppercase;
	}

	.logo:hover {
		color: var(--primary-hover);
	}

	.theme-toggle {
		background: none;
		border: none;
		cursor: pointer;
		padding: 0.5rem;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--primary-color);
		border-radius: 0.375rem;
		transition: all 0.2s ease;
		min-width: 40px;
		min-height: 40px;
	}

	.mobile-menu-toggle {
		background: none;
		border: none;
		cursor: pointer;
		padding: 0.5rem 1rem;
		display: none;
		align-items: center;
		justify-content: center;
		color: var(--primary-color);
		border-radius: 0.375rem;
		transition: all 0.2s ease;
		font-size: 10px;
		font-weight: 500;
		letter-spacing: 0.5px;
		text-transform: uppercase;
		min-height: 40px;
	}

	.mobile-menu-toggle:hover,
	.theme-toggle:hover {
		background: var(--hover-bg);
		color: var(--primary-hover);
		transform: scale(1.05);
	}

	.theme-toggle:hover {
		background: var(--secondary-bg);
		color: var(--primary-hover);
		transform: scale(1.05);
	}

	.mobile-menu-toggle:active,
	.theme-toggle:active {
		transform: scale(0.95);
	}

	.theme-toggle svg {
		transition: transform 0.2s ease;
	}

	.theme-toggle:hover svg {
		transform: rotate(15deg);
	}

	/* Mobile responsive */
	@media (max-width: 768px) {
		.main-nav {
			padding: 0.75rem 1rem;
			flex-direction: row;
			gap: 0;
		}

		.nav-left {
			justify-content: flex-start;
		}

		.nav-right {
			justify-content: flex-end;
		}

		/* Show mobile menu button, hide desktop nav links */
		.mobile-menu-toggle {
			display: flex;
		}

		.desktop-nav-links {
			display: none;
		}

		.nav-center {
			position: absolute;
			left: 50%;
			transform: translateX(-50%);
			z-index: 10;
		}

		.logo {
			font-size: 1.5rem;
		}
	}

	@media (max-width: 480px) {
		.main-nav {
			padding: 0.5rem 1rem;
		}

		.logo {
			font-size: 1.25rem;
		}

		.theme-toggle,
		.mobile-menu-toggle {
			min-width: auto;
			min-height: 36px;
			padding: 0.375rem;
		}
		
		.mobile-menu-toggle {
			padding: 0.375rem 0.75rem;
			font-size: 10px;
		}
=======
	import { page } from '$app/state';
	import logo from '$lib/images/svelte-logo.svg';
	import github from '$lib/images/github.svg';
</script>

<header>
	<div class="corner">
		<a href="https://svelte.dev/docs/kit">
			<img src={logo} alt="SvelteKit" />
		</a>
	</div>

	<nav>
		<svg viewBox="0 0 2 3" aria-hidden="true">
			<path d="M0,0 L1,2 C1.5,3 1.5,3 2,3 L2,0 Z" />
		</svg>
		<ul>
			<li aria-current={page.url.pathname === '/' ? 'page' : undefined}>
				<a href="/">Home</a>
			</li>
			<li aria-current={page.url.pathname === '/about' ? 'page' : undefined}>
				<a href="/about">About</a>
			</li>
			<li aria-current={page.url.pathname.startsWith('/sverdle') ? 'page' : undefined}>
				<a href="/sverdle">Sverdle</a>
			</li>
		</ul>
		<svg viewBox="0 0 2 3" aria-hidden="true">
			<path d="M0,0 L0,3 C0.5,3 0.5,3 1,2 L2,0 Z" />
		</svg>
	</nav>

	<div class="corner">
		<a href="https://github.com/sveltejs/kit">
			<img src={github} alt="GitHub" />
		</a>
	</div>
</header>

<style>
	header {
		display: flex;
		justify-content: space-between;
	}

	.corner {
		width: 3em;
		height: 3em;
	}

	.corner a {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 100%;
		height: 100%;
	}

	.corner img {
		width: 2em;
		height: 2em;
		object-fit: contain;
	}

	nav {
		display: flex;
		justify-content: center;
		--background: rgba(255, 255, 255, 0.7);
	}

	svg {
		width: 2em;
		height: 3em;
		display: block;
	}

	path {
		fill: var(--background);
	}

	ul {
		position: relative;
		padding: 0;
		margin: 0;
		height: 3em;
		display: flex;
		justify-content: center;
		align-items: center;
		list-style: none;
		background: var(--background);
		background-size: contain;
	}

	li {
		position: relative;
		height: 100%;
	}

	li[aria-current='page']::before {
		--size: 6px;
		content: '';
		width: 0;
		height: 0;
		position: absolute;
		top: 0;
		left: calc(50% - var(--size));
		border: var(--size) solid transparent;
		border-top: var(--size) solid var(--color-theme-1);
	}

	nav a {
		display: flex;
		height: 100%;
		align-items: center;
		padding: 0 0.5rem;
		color: var(--color-text);
		font-weight: 700;
		font-size: 0.8rem;
		text-transform: uppercase;
		letter-spacing: 0.1em;
		text-decoration: none;
		transition: color 0.2s linear;
	}

	a:hover {
		color: var(--color-theme-1);
>>>>>>> origin/master
	}
</style>
