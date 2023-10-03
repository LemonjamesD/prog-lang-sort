<script>
export let className = "";
</script>

<div class = {className}>
    <slot/>
</div>

<style>
div {
    background-color: var(--background-color, #FFFFFF);
}
</style>