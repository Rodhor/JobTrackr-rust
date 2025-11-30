<script lang="ts">
    import ReminderForm from "$lib/components/forms/ReminderForm.svelte";
    export let params;
</script>

<ReminderForm reminderID={Number(params.id)} />
