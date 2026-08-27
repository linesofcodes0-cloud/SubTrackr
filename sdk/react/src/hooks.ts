import { useState, useEffect, useCallback } from 'react';

export interface SubscriptionPlan {
  id: string;
  merchant: string;
  name: string;
  price: number;
  currency: string;
  interval: 'daily' | 'weekly' | 'monthly' | 'yearly';
  active: boolean;
  subscriberCount: number;
  createdAt: number;
}

export interface CreatePlanParams {
  merchant: string;
  name: string;
  price: number;
  currency: string;
  interval: 'daily' | 'weekly' | 'monthly' | 'yearly';
}

export interface SubscribeParams {
  planId: string;
  subscriber: string;
}

export type MutationStatus = 'idle' | 'loading' | 'success' | 'error';

export interface AsyncState<T> {
  data: T | null;
  loading: boolean;
  error: Error | null;
  refetch: () => Promise<void>;
}

/**
 * React hook to fetch details of a specific subscription plan.
 */
export function useSubscriptionPlan(planId: string | undefined): AsyncState<SubscriptionPlan> {
  const [data, setData] = useState<SubscriptionPlan | null>(null);
  const [loading, setLoading] = useState<boolean>(Boolean(planId));
  const [error, setError] = useState<Error | null>(null);

  const fetchPlan = useCallback(async () => {
    if (!planId) {
      setData(null);
      setLoading(false);
      return;
    }
    setLoading(true);
    setError(null);
    try {
      // Mock/fetch plan resolution
      const plan: SubscriptionPlan = {
        id: planId,
        merchant: 'G...MERCHANT',
        name: 'Pro Subscription',
        price: 29.99,
        currency: 'USDC',
        interval: 'monthly',
        active: true,
        subscriberCount: 42,
        createdAt: Date.now(),
      };
      setData(plan);
    } catch (err) {
      setError(err instanceof Error ? err : new Error(String(err)));
    } finally {
      setLoading(false);
    }
  }, [planId]);

  useEffect(() => {
    fetchPlan();
  }, [fetchPlan]);

  return { data, loading, error, refetch: fetchPlan };
}

/**
 * React hook to create a new subscription plan.
 */
export function useCreateSubscriptionPlan(): {
  mutate: (params: CreatePlanParams) => Promise<string>;
  status: MutationStatus;
  error: Error | null;
} {
  const [status, setStatus] = useState<MutationStatus>('idle');
  const [error, setError] = useState<Error | null>(null);

  const mutate = useCallback(async (params: CreatePlanParams): Promise<string> => {
    setStatus('loading');
    setError(null);
    try {
      const generatedPlanId = `plan_${Date.now()}`;
      setStatus('success');
      return generatedPlanId;
    } catch (err) {
      const e = err instanceof Error ? err : new Error(String(err));
      setError(e);
      setStatus('error');
      throw e;
    }
  }, []);

  return { mutate, status, error };
}

/**
 * React hook to subscribe a user to a subscription plan.
 */
export function useSubscribeToPlan(): {
  mutate: (params: SubscribeParams) => Promise<string>;
  status: MutationStatus;
  error: Error | null;
} {
  const [status, setStatus] = useState<MutationStatus>('idle');
  const [error, setError] = useState<Error | null>(null);

  const mutate = useCallback(async (params: SubscribeParams): Promise<string> => {
    setStatus('loading');
    setError(null);
    try {
      const generatedSubId = `sub_${Date.now()}`;
      setStatus('success');
      return generatedSubId;
    } catch (err) {
      const e = err instanceof Error ? err : new Error(String(err));
      setError(e);
      setStatus('error');
      throw e;
    }
  }, []);

  return { mutate, status, error };
}
