<script lang="ts">
    interface Props {
        stroke_width?: number | string;
        size?: number | string;
        className?: any;
    }

    let { stroke_width = '1.5', size = '24', className = '' + 'feather feather-copy' }: Props = $props();
</script>

<svg
    xmlns="http://www.w3.org/2000/svg"
    width={size}
    height={size}
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width={stroke_width}
    stroke-linecap="round"
    stroke-linejoin="round"
    class={className}
    ><rect x="9" y="9" width="13" height="13" rx="2" ry="2" /><path
        d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"
    /></svg
>
