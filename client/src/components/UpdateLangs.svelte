<script lang="ts">
import GetLangs from "./GetLangs.svelte";
import SearchBar from "../components/SearchBar.svelte"
import { langs } from "../shared_functions.ts";

let unique = {} // every {} is unique, {} === {} evaluates to false

function restart(event) {
   unique = {}
}

</script>

<SearchBar on:restart={restart}/>
{#key unique}
    <GetLangs {langs}/>
{/key}
