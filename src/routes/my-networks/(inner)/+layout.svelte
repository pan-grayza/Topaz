<script lang="ts">
    interface Props {
        children?: import('svelte').Snippet;
    }

    let { children }: Props = $props();
</script>

<div class="relative flex flex-col w-full h-full p-4">
    <a href="/my-networks">Go back</a>
    {@render children?.()}
</div>
