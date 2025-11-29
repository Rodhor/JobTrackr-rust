<script lang="ts">
    import { page } from "$app/stores";
    import PersonForm from "$lib/components/forms/PersonForm.svelte";
    const caller = $page.url.searchParams.get("caller");
</script>

<PersonForm {caller} />
