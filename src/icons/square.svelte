<script lang="ts">
    interface Props {
        stroke_width?: number | string;
        size?: number | string;
        className?: any;
    }

    let { stroke_width = '1.5', size = '24', className = '' + 'feather feather-square' }: Props = $props();
</script>

<svg
    xmlns="http://www.w3.org/2000/svg"
    width={size}
    height={size}
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    class={className}
    stroke-width={stroke_width}
    stroke-linecap="round"
    stroke-linejoin="round"
    ><rect x="3" y="3" width="18" height="18" rx="2" ry="2" /></svg
>
