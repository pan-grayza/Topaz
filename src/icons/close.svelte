<script lang="ts">
    interface Props {
        stroke_width?: number | string;
        size?: number | string;
        className?: any;
    }

    let { stroke_width = '1.25', size = '24', className = '' + 'feather feather-x' }: Props = $props();
</script>

<svg
    xmlns="http://www.w3.org/2000/svg"
    width={size}
    height={size}
    viewBox="0 0 24 24"
    fill="none"
    class={className}
    stroke="currentColor"
    stroke-width={stroke_width}
    stroke-linecap="round"
    stroke-linejoin="round"
    ><line x1="18" y1="6" x2="6" y2="18"></line><line
        x1="6"
        y1="6"
        x2="18"
        y2="18"
    ></line>
</svg>
