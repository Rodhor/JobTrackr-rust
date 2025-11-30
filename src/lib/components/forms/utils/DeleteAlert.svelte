<script lang="ts">
    import * as AlertDialog from "$lib/components/ui/alert-dialog/index.js";
    import { buttonVariants } from "$lib/components/ui/button/index.js";
    import Trash2Icon from "lucide-svelte/icons/trash-2";

    let {
        objectText = "",
        description = "",
        onDelete = () => {},
    }: {
        objectText: string;
        description?: string;
        onDelete?: () => void;
    } = $props();

    function handleConfirm() {
        onDelete();
    }
</script>

<AlertDialog.Root>
    <AlertDialog.Trigger
        class={buttonVariants({ variant: "destructive", size: "sm" })}
    >
        <Trash2Icon class="size-4" />
        Delete
    </AlertDialog.Trigger>
    <AlertDialog.Content>
        <AlertDialog.Header>
            <AlertDialog.Title>
                Are you sure you want to delete {objectText}?
            </AlertDialog.Title>
            {#if description}
                <AlertDialog.Description>
                    {description}
                </AlertDialog.Description>
            {/if}
        </AlertDialog.Header>
        <AlertDialog.Footer>
            <AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
            <AlertDialog.Action onclick={handleConfirm}>
                Delete
            </AlertDialog.Action>
        </AlertDialog.Footer>
    </AlertDialog.Content>
</AlertDialog.Root>
