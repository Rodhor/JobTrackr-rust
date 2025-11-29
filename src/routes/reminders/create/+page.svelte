<script lang="ts">
    import { page } from "$app/stores";
    import ReminderForm from "$lib/components/forms/ReminderForm.svelte";
    const caller = $page.url.searchParams.get("caller");
</script>

<ReminderForm {caller} />
