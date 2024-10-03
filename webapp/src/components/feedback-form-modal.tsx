'use client';

import { Dialog, DialogContent, DialogDescription, DialogTrigger, DialogHeader, DialogTitle } from './ui/dialog';
import { FeedbackForm } from './feedback-form';
import FeedbackButton from './feedback-button';
import { useState } from 'react';


export function FeedbackFormModal() {
  const [isFeedbackModalOpen, setFeedbackModalOpen] = useState(false);

  return (
    <Dialog open={isFeedbackModalOpen} onOpenChange={setFeedbackModalOpen}>
      <DialogTrigger>
        <FeedbackButton />
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Send Feedback</DialogTitle>
          <DialogDescription>
            Help us improve by sharing your feedback.
          </DialogDescription>
        </DialogHeader>
        <div className='max-w-full'>
          <FeedbackForm onSubmit={() => setFeedbackModalOpen(false)} />
        </div>
      </DialogContent>
    </Dialog>
  );
}
