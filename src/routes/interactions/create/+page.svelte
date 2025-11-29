<script lang="ts">
    import { page } from "$app/stores";
    import InteractionForm from "$lib/components/forms/InteractionForm.svelte";
    const caller = $page.url.searchParams.get("caller");
</script>

<InteractionForm {caller} />
