export interface Reminder {
  id: number;
  title: string;
  message?: string;
  reminderDate: string;
  isCompleted: boolean;
  applicationId?: number;
  jobListingId?: number;
  interactionId?: number;
  noteId?: number;
  companyId?: number;
  personId?: number;
  createdAt: string;
  updatedAt: string;
}
