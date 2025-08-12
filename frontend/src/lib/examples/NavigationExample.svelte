<!-- Example usage of the DRY BaseNavLink component -->
<script>
  import BaseNavLink from '$lib/elements/BaseNavLink.svelte';
</script>

<!-- Different variants and sizes -->
<nav>
  <BaseNavLink href="/" active={true}>Home</BaseNavLink>
  <BaseNavLink href="/about" variant="secondary">About</BaseNavLink>
  <BaseNavLink href="/contact" size="lg">Contact</BaseNavLink>
  <BaseNavLink href="/blog" variant="secondary" size="sm">Blog</BaseNavLink>
</nav>

<!-- You can also create other components using the same classes -->
<button class="btn-base variant-primary size-md">
  <span class="underline-animation">Button with same animation</span>
</button>

<div class="custom-element variant-secondary">
  <span class="underline-animation">Custom element with animation</span>
</div>

<style>
  .custom-element {
    /* Inherits all the base styles and can be extended */
    border: 1px solid var(--secondary-color);
  }
</style>
