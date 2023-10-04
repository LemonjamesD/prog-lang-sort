<script lang="ts">
import ColoredBox from "./ColoredBox.svelte";
import ColoredText from "./ColoredText.svelte";
import { pastel_color } from "../shared_functions.ts";

export let langs: Promise<JSON[]>;
</script>

<div>
    {#await langs}
        Loading....
    {:then langs}
        <!-- There's probably a better way to do this --> 
        {#each langs as lang}
            <ColoredBox className="lang" --background-color={pastel_color()}>
                <p> {lang.display} 
                    {#if lang.display.toLowerCase() != lang.name.toLowerCase()}
                        ({lang.name})
                    {/if} 
                </p>
                {#each lang.types as type}
                    <ColoredText className="category">{type}</ColoredText>
                {/each}
            </ColoredBox>
        {/each}
    {:catch error}
        System error: {error.message}
    {/await}
</div>

<style>
@import url('https://fonts.cdnfonts.com/css/handwritingcr');
:global(.category) {
    inline-size: max-content;
    overflow-wrap: anywhere;
    display: inline-block;
    margin: 2px;
    -webkit-text-stroke-width: 1px;
    -webkit-text-stroke-color: black;
}
:global(.lang) {
    aspect-ratio: 1;
    font-family: "HandwritingCR", sans-serif;
    font-weight: bolder;
    color: #575757;
    padding-left: 10px;
    padding-right: 10px;
    padding-top: 4px;
    padding-bottom: 6px;
    margin: 10px;
    box-shadow: 5px 5px;
    background-size: cover;
    background-image: url("https://static.vecteezy.com/system/resources/previews/018/760/469/original/crumpled-paper-texture-png.png");

    max-width: 50vw;

    inline-size: max-content;
    display: inline-block;
}
</style>
