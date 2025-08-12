<script lang="ts">
  export let href: string = '#';
  export let active: boolean = false;
  export let target: string | undefined = undefined;
  export let rel: string | undefined = undefined;
  export let variant: 'primary' | 'secondary' = 'primary';
  export let size: 'sm' | 'md' | 'lg' = 'md';
  export let id: string | undefined = undefined;
  export let testId: string | undefined = undefined;
</script>

<a
  {id}
  class="btn-base variant-{variant} size-{size} {active ? 'active' : ''}"
  data-testid={testId}
  data-variant={variant}
  data-active={active}
  href={href}
  color={variant === 'primary' ? 'var(--primary-text)' : 'var(--secondary-color)'}
  {target}
  {rel}
>
  <span class="underline-animation {active ? 'active' : ''}">
    <slot />
  </span>
</a>

<style>
/* Import only component styles - theme.css is imported globally in app.css */
@import '../styles/components.css';

/* Enhanced active state - uses improved theme variables */
.active {
  background: var(--active-bg) !important;
  color: var(--primary-hover) !important;
  font-weight: 600;
}
</style>
