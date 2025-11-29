<script lang="ts">
    import { page } from "$app/stores";
    import ApplicationForm from "$lib/components/forms/ApplicationForm.svelte";
    const caller = $page.url.searchParams.get("caller");
</script>

<ApplicationForm {caller} />
