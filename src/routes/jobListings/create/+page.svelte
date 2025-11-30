<script lang="ts">
    import { page } from "$app/state";
    import JobListingForm from "$lib/components/forms/JoblistingForm.svelte";

    const callerChainParam = page.url.searchParams.get("callerChain");

    const callerChain =
        callerChainParam && callerChainParam.trim()
            ? callerChainParam.split(",").filter(Boolean)
            : [];
</script>

<JobListingForm {callerChain} />
