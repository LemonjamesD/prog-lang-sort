<script lang="ts">
import Fuse from 'fuse.js'
import { get_langs, langs, all_langs, refresh_langs } from "../shared_functions.ts";
import { createEventDispatcher } from 'svelte';

const dispatch = createEventDispatcher();

async function search_name() {
    if (value == "") {
        refresh_langs(all_langs);
        return;
    }

    const awaited_langs: JSON[] = await all_langs;
    const fuse = new Fuse(awaited_langs, {
        keys: ["display"]
    });
    refresh_langs(fuse.search(value).map((x) => x.item));
    dispatch("restart", {});
}

let value;
</script>

<input on:input = {search_name} bind:value={value} placeholder="Type here to search..."/>
