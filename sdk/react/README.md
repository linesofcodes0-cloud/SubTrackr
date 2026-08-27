# SubTrackr React Hooks SDK

React hooks for integrating SubTrackr subscription plans, plan creation, and user subscription flows into React web & React Native applications.

## Installation

```bash
npm install @subtrackr/react
```

## Available Hooks

### `useSubscriptionPlan`

Fetches details and status of a subscription plan by ID.

```tsx
import { useSubscriptionPlan } from '@subtrackr/react';

function PlanDetails({ planId }: { planId: string }) {
  const { data: plan, loading, error, refetch } = useSubscriptionPlan(planId);

  if (loading) return <div>Loading plan...</div>;
  if (error) return <div>Error loading plan: {error.message}</div>;
  if (!plan) return <div>Plan not found</div>;

  return (
    <div>
      <h3>{plan.name}</h3>
      <p>Price: ${plan.price} / {plan.interval}</p>
      <p>Active Subscribers: {plan.subscriberCount}</p>
      <button onClick={refetch}>Refresh</button>
    </div>
  );
}
```

### `useCreateSubscriptionPlan`

Mutation hook to create a new merchant subscription plan.

```tsx
import { useCreateSubscriptionPlan } from '@subtrackr/react';

function CreatePlanForm() {
  const { mutate: createPlan, status, error } = useCreateSubscriptionPlan();

  const handleCreate = async () => {
    try {
      const planId = await createPlan({
        merchant: 'G...MERCHANT',
        name: 'Enterprise Plan',
        price: 99.99,
        currency: 'USDC',
        interval: 'monthly',
      });
      console.log('Created plan:', planId);
    } catch (err) {
      console.error('Failed to create plan', err);
    }
  };

  return (
    <button onClick={handleCreate} disabled={status === 'loading'}>
      {status === 'loading' ? 'Creating...' : 'Create Plan'}
    </button>
  );
}
```

### `useSubscribeToPlan`

Mutation hook to subscribe a user to an active subscription plan.

```tsx
import { useSubscribeToPlan } from '@subtrackr/react';

function SubscribeButton({ planId, userAddress }: { planId: string; userAddress: string }) {
  const { mutate: subscribe, status, error } = useSubscribeToPlan();

  const handleSubscribe = async () => {
    try {
      const subscriptionId = await subscribe({
        planId,
        subscriber: userAddress,
      });
      alert(`Successfully subscribed! Subscription ID: ${subscriptionId}`);
    } catch (err) {
      console.error('Subscription failed', err);
    }
  };

  return (
    <button onClick={handleSubscribe} disabled={status === 'loading'}>
      {status === 'loading' ? 'Subscribing...' : 'Subscribe Now'}
    </button>
  );
}
```
