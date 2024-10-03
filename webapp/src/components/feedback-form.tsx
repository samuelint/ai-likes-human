'use client';

import { zodResolver } from '@hookform/resolvers/zod';
import { useForm } from 'react-hook-form';
import { z } from 'zod';
import { toast } from './ui/use-toast';
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from './ui/form';
import { Input, } from './ui/input';
import { Button } from './ui/button';
import { Textarea } from './ui/textarea';
import * as Sentry from '@sentry/react';

const optionalEmail = z.union( [
  z.literal( '' ),
  z.string().email({ message: 'Please provide a valid email address.' }),
] ).optional();

const FormSchema = z.object({
  name: z.string().optional(),
  email: optionalEmail,
  description: z.string().min(10, { message: 'Please provide a description of at least 10 characters.' }),
});

interface Props {
  onSubmit?: (data: z.infer<typeof FormSchema>) => void;
}

export function FeedbackForm({ onSubmit }: Props) {
  const form = useForm<z.infer<typeof FormSchema>>({
    resolver: zodResolver(FormSchema),
    defaultValues: {
      name: '',
      email: '',
      description: '',
    },
  });

  function handleSubmit(data: z.infer<typeof FormSchema>) {
    Sentry.captureFeedback(
      {
        name: data.name,
        email: data.email,
        message: data.description,
      },
      {
        includeReplay: false,
        attachments: [],
      },
    );

    toast({
      title: 'Feedback sent!',
    });

    if (onSubmit) {
      onSubmit(data);
    }
  }

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(handleSubmit)} className="">
        <FormField
          control={form.control}
          name="name"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Name</FormLabel>
              <FormControl>
                <Input placeholder="Full Name" {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="email"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Email</FormLabel>
              <FormControl>
                <Input placeholder="Email" {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="description"
          render={({ field }) => (
            <FormItem>
              <FormLabel>{'Description (Required)'}</FormLabel>
              <FormControl>
                <Textarea placeholder="What could be done better? Did you have a bug?" {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <Button type="submit">Submit</Button>
      </form>
    </Form>
  );
}
