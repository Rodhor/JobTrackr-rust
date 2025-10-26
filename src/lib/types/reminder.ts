export interface Reminder {
  id: number;
  contactId?: number;
  jobListingId?: number;
  applicationId?: number;
  personId?: number;
  companyId?: number;
  noteId: number;
  reminderDate?: string;
  title?: string;
  message?: string;
  isCompleted?: Boolean;
  createAt: string;
  updatedAt: string;
}
