<script lang="ts">
    import { page } from "$app/state";
    import ApplicationForm from "$lib/components/forms/ApplicationForm.svelte";

    const callerChainParam = page.url.searchParams.get("callerChain") || "";
    const callerChain = callerChainParam
        ? callerChainParam.split(",").filter(Boolean)
        : [];
</script>

<ApplicationForm {callerChain} />
