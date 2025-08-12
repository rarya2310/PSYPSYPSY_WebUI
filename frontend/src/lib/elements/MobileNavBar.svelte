<script lang="ts">
  import { page } from '$app/stores';
  import BaseNavLink from './BaseNavLink.svelte';
  
  export let isOpen: boolean = false;
  
  // Close dropdown when clicking outside
  function handleClickOutside(event: MouseEvent) {
    const target = event.target as HTMLElement;
    if (!target.closest('.mobile-nav-dropdown') && !target.closest('.mobile-menu-toggle')) {
      isOpen = false;
    }
  }
  
  // Close dropdown on route change
  $: if ($page.url.pathname) {
    isOpen = false;
  }
</script>

<svelte:window on:click={handleClickOutside} />

{#if isOpen}
  <div class="mobile-nav-dropdown">
    <div class="mobile-nav-content">
      <BaseNavLink href="/apparels" active={$page.url.pathname.toString() === '/apparels'} variant="primary">APPARELS</BaseNavLink>
      <BaseNavLink href="/collections" active={$page.url.pathname.toString() === '/collections'} variant="primary">COLLECTIONS</BaseNavLink>
      <BaseNavLink href="/new-in" active={$page.url.pathname.toString() === '/new-in'} variant="primary">NEW IN</BaseNavLink>
    </div>
  </div>
{/if}

<style>
  .mobile-nav-dropdown {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    background: var(--nav-bg);
    border-top: 1px solid var(--hover-bg);
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
    z-index: 50;
    animation: slideDown 0.2s ease-out;
  }
  
  .mobile-nav-content {
    display: flex;
    flex-direction: row;
    padding: 0.5rem 1rem;
    gap: 1rem;
    justify-content: center;
  }
  
  .mobile-nav-content :global(a) {
    padding: 0.5rem 0.75rem;
    text-align: center;
    border-radius: var(--border-radius);
    font-weight: 500;
    letter-spacing: 0.5px;
    font-size: 10px;
    text-transform: uppercase;
  }
  
  @keyframes slideDown {
    from {
      opacity: 0;
      transform: translateY(-10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style>
