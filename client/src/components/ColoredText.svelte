<script>
export let className = "";
</script>

<p class={className}>
    <slot/>
</p>

<style>
p {
    color: var(--color, #FFFFFF);
}
</style>
