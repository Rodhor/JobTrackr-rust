<script lang="ts">
    import { onMount } from "svelte";
    import { Button } from "$lib/components/ui/button";
    import { Badge } from "$lib/components/ui/badge";
    import PersonDialog from "$lib/components/formDialogs/PersonDialog.svelte";
    import { people, loadPeople, deletePerson } from "$lib/stores/people";
    import { writable } from "svelte/store";
    import { Role, RoleDisplay } from "$lib/types/enums";
    import type { Person } from "$lib/types/person";

    const dialogOpen = writable(false);
    const mode = writable<"create" | "edit">("create");
    const selectedPerson = writable<Person | null>(null);

    onMount(loadPeople);

    function handleCreate() {
        selectedPerson.set(null);
        mode.set("create");
        dialogOpen.set(true);
    }

    function handleEdit(person: Person) {
        selectedPerson.set(person);
        mode.set("edit");
        dialogOpen.set(true);
    }

    async function handleDelete(id: number) {
        try {
            await deletePerson(id);
        } catch (err) {
            console.error("Failed to delete person:", err);
        }
    }

    function roleColor(role?: Role) {
        switch (role) {
            case Role.Recruiter:
                return "bg-blue-100 text-blue-800";
            case Role.HiringManager:
                return "bg-green-100 text-green-800";
            case Role.TeamLead:
                return "bg-yellow-100 text-yellow-800";
            case Role.Developer:
                return "bg-purple-100 text-purple-800";
            default:
                return "bg-muted text-foreground";
        }
    }
</script>

<!-- ----------------------------------------------------------
Header
----------------------------------------------------------- -->
<div class="mb-6 flex items-center justify-between">
    <h1 class="text-2xl font-semibold tracking-tight">People</h1>
    <Button onclick={handleCreate}>New Person</Button>
</div>

<PersonDialog
    bind:open={$dialogOpen}
    mode={$mode}
    existingPerson={$selectedPerson}
/>

<!-- ----------------------------------------------------------
Table
----------------------------------------------------------- -->
<div
    class="overflow-hidden rounded-lg border border-border bg-background shadow-sm"
>
    <table class="min-w-full text-sm">
        <thead
            class="bg-muted/50 text-left text-xs uppercase tracking-wider text-muted-foreground"
        >
            <tr>
                <th class="px-4 py-3">Name</th>
                <th class="px-4 py-3">Email</th>
                <th class="px-4 py-3">Phone</th>
                <th class="px-4 py-3">Role</th>
                <th class="px-4 py-3">LinkedIn</th>
                <th class="px-4 py-3 text-right">Actions</th>
            </tr>
        </thead>

        <tbody>
            {#each $people as p (p.id)}
                <tr class="border-t hover:bg-muted/30 transition-colors">
                    <td class="px-4 py-3 font-medium text-foreground">
                        {p.firstName}
                        {p.lastName}
                    </td>
                    <td class="px-4 py-3 text-muted-foreground">
                        {p.email || "—"}
                    </td>
                    <td class="px-4 py-3 text-muted-foreground">
                        {p.phoneNumber || "—"}
                    </td>
                    <td class="px-4 py-3">
                        <Badge class={roleColor(p.role)}>
                            {p.role ? RoleDisplay[p.role] : "—"}
                        </Badge>
                    </td>
                    <td
                        class="px-4 py-3 text-muted-foreground truncate max-w-[200px]"
                    >
                        {#if p.linkedinUrl}
                            <a
                                href={p.linkedinUrl}
                                class="text-blue-600 hover:underline"
                                target="_blank"
                            >
                                {p.linkedinUrl}
                            </a>
                        {:else}
                            —
                        {/if}
                    </td>
                    <td class="px-4 py-3 text-right flex justify-end gap-2">
                        <Button
                            size="sm"
                            variant="outline"
                            onclick={() => handleEdit(p)}>Edit</Button
                        >
                        <Button
                            size="sm"
                            variant="destructive"
                            onclick={() => handleDelete(p.id)}>Delete</Button
                        >
                    </td>
                </tr>
            {/each}

            {#if $people.length === 0}
                <tr>
                    <td
                        colspan="6"
                        class="px-4 py-10 text-center text-sm text-muted-foreground"
                    >
                        No people yet.
                    </td>
                </tr>
            {/if}
        </tbody>
    </table>
</div>
