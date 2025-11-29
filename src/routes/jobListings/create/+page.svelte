<script lang="ts">
    import { page } from "$app/stores";
    import JobListingForm from "$lib/components/forms/JoblistingForm.svelte";
    const caller = $page.url.searchParams.get("caller");
</script>

<JobListingForm {caller} />
