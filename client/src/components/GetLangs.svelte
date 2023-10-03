<script lang="ts">
import ColoredBox from "./ColoredBox.svelte";
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
                <p> {lang.display} ({lang.name}) </p>
                <div>
                    {#each lang.types as type}
                        <ColoredBox className="category" --background-color={pastel_color()}>
                            {type}
                        </ColoredBox>
                    {/each}
                </div>
            </ColoredBox>
        {/each}
    {:catch error}
        System error: {error.message}
    {/await}
</div>

<style>
:global(.category) {
    border-radius: 10px;
    margin: 5px;
    padding-left: 10px;
    padding-right: 10px;
    padding-top: 3px;
    padding-bottom: 3px;
    border: 3px;
    border-style: solid;
    border-color: #9faabd;
    text-wrap: balance;
    inline-size: max-content;
    display: inline-block;
    
    box-shadow: 5px 5px;
}
:global(.lang) {
    color: #575757;
    border-radius: 10px;
    padding-left: 10px;
    padding-right: 10px;
    padding-top: 4px;
    padding-bottom: 6px;
    margin: 10px;
    box-shadow: 5px 5px;

    inline-size: max-content;
    display: inline-block;
}
</style>