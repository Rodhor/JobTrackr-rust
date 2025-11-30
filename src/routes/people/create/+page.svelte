<script lang="ts">
    import { page } from "$app/state";
    import PersonForm from "$lib/components/forms/PersonForm.svelte";

    const callerChainParam = page.url.searchParams.get("callerChain") || "";
    const callerChain = callerChainParam
        ? callerChainParam.split(",").filter(Boolean)
        : [];
</script>

<PersonForm {callerChain} />
