<script lang="ts">
    import { page } from "$app/state";
    import JobListingForm from "$lib/components/forms/JoblistingForm.svelte";

    const callerChainParam = page.url.searchParams.get("callerChain");
    console.log("Route: callerChainParam raw:", callerChainParam);

    const callerChain =
        callerChainParam && callerChainParam.trim()
            ? callerChainParam.split(",").filter(Boolean)
            : [];

    console.log("Route: parsed callerChain:", callerChain);
</script>

<JobListingForm {callerChain} />
