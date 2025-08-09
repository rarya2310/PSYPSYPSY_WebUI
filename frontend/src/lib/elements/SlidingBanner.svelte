<script lang="ts">
  export let messages: string[] = [
    'PSYPSYPSY',
    'It\'s a Vice.',
    'Shipping all over India.',
    'Hypnosis in thread.'
  ];
  export let interval: number = 2500; // ms
  export let bg: string | undefined = undefined; // custom background
  
  let current = 0;
  let timer: ReturnType<typeof setInterval>;
  let bannerHeight = 20; // default height

  function next() {
    current = (current + 1) % messages.length;
  }

  function updateHeight() {
    if (typeof window !== 'undefined') {
      const width = window.innerWidth;
      if (width <= 320) {
        bannerHeight = 12;
      } else if (width <= 480) {
        bannerHeight = 14;
      } else if (width <= 768) {
        bannerHeight = 16;
      } else if (width <= 1024) {
        bannerHeight = 18;
      } else {
        bannerHeight = 20;
      }
    }
  }

  import { onMount, onDestroy } from 'svelte';
  
  onMount(() => {
    updateHeight();
    timer = setInterval(next, interval);
    
    if (typeof window !== 'undefined') {
      window.addEventListener('resize', updateHeight);
    }
  });
  
  onDestroy(() => {
    if (timer) {
      clearInterval(timer);
    }
    if (typeof window !== 'undefined') {
      window.removeEventListener('resize', updateHeight);
    }
  });
</script>

<div class="sliding-banner" style={bg ? `background: ${bg}` : ''}>
  <div class="banner-viewport">
    <div class="banner-messages" style="transform: translateY(-{current * bannerHeight}px);">
      {#each messages as msg}
        <div class="banner-message">{msg}</div>
      {/each}
    </div>
  </div>
</div>

<style>

.sliding-banner {
  width: 100%;
  background: var(--sliding-banner-bg);
  color: var(--primary-sliding-banner-text);
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 20px;
  border-radius: 0;
  overflow: hidden;
  font-family: inherit;
  position: relative;
  box-shadow: none;
}

.banner-viewport {
  height: 20px;
  width: 100%;
  overflow: hidden;
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
}

.banner-messages {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: auto;
  display: flex;
  flex-direction: column;
  transition: transform 0.5s cubic-bezier(0.25, 0.46, 0.45, 0.94);
  will-change: transform;
}

.banner-message {
  height: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  font-size: 10px;
  font-weight: 600;
  letter-spacing: 0.5px;
  padding: 0 1rem;
  background: transparent;
  color: var(--primary-sliding-banner-text);
  text-align: center;
  box-sizing: border-box;
  flex-shrink: 0;
  white-space: nowrap;
  text-transform: uppercase;
}

/* Tablet styles */
@media (max-width: 1024px) {
  .sliding-banner {
    min-height: 18px;
  }
  .banner-viewport {
    height: 18px;
  }
  .banner-message {
    height: 18px;
    font-size: 9px;
    padding: 0 0.8rem;
  }
}

/* Mobile styles */
@media (max-width: 768px) {
  .sliding-banner {
    min-height: 16px;
  }
  .banner-viewport {
    height: 16px;
  }
  .banner-message {
    height: 16px;
    font-size: 8px;
    padding: 0 0.6rem;
    letter-spacing: 0.3px;
  }
}

/* Small mobile styles */
@media (max-width: 480px) {
  .sliding-banner {
    min-height: 14px;
  }
  .banner-viewport {
    height: 14px;
  }
  .banner-message {
    height: 14px;
    font-size: 7px;
    padding: 0 0.4rem;
    letter-spacing: 0.2px;
  }
}

/* Extra small mobile styles */
@media (max-width: 320px) {
  .sliding-banner {
    min-height: 12px;
  }
  .banner-viewport {
    height: 12px;
  }
  .banner-message {
    height: 12px;
    font-size: 6px;
    padding: 0 0.3rem;
    letter-spacing: 0.1px;
  }
}
</style>
