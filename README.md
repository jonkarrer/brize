# Brize

What would it look like to bootstrap a SaaS application using Rust?

## Features

... 🚧

## Tech Stack

- **Framework**: [Dioxus](https://dioxuslabs.com/)
- **Database**: [Postgres](https://www.postgresql.org/)
- **ORM**: [SeaORM](https://www.sea-ql.org/SeaORM/)
- **Payments**: [Stripe](https://stripe.com/)
- **UI Library**: ...🚧

## Getting Started

```bash
git clone https://github.com/jonkarrer/brize.git
cd brize
```

## Running Locally

[Install](https://docs.stripe.com/stripe-cli) and log in to your Stripe account:

```bash
stripe login
```

Use the included setup crate to create your `.env` file:

```bash
cargo run -p setup
```

Run the database migrations and seed the database with a default user, and optionally a default team:

```bash
cargo run -p migrate
cargo run -p seed
```

This will create the following user and team:

- User: `test@test.com`
- Password: `admin123`

You can also create new users through the `/sign-up` route.

Finally, run the development server:

```bash
cargo run
```

Open [http://localhost:3000](http://localhost:3000) in your browser to see the app in action.

You can listen for Stripe webhooks locally through their CLI to handle subscription change events:

```bash
stripe listen --forward-to localhost:3000/api/stripe/webhook
```

## Testing Payments

To test Stripe payments, use the following test card details:

- Card Number: `4242 4242 4242 4242`
- Expiration: Any future date
- CVC: Any 3-digit number

## Going to Production

When you're ready to deploy your SaaS application to production, follow these steps:

### Set up a production Stripe webhook

1. Go to the Stripe Dashboard and create a new webhook for your production environment.
2. Set the endpoint URL to your production API route (e.g., `https://yourdomain.com/api/stripe/webhook`).
3. Select the events you want to listen for (e.g., `checkout.session.completed`, `customer.subscription.updated`).

### Deploy to something

1. Push your code to a GitHub repository.

...🚧

### Add environment variables

In your Vercel project settings (or during deployment), add all the necessary environment variables. Make sure to update the values for the production environment, including:

1. `BASE_URL`: Set this to your production domain.
2. `STRIPE_SECRET_KEY`: Use your Stripe secret key for the production environment.
3. `STRIPE_WEBHOOK_SECRET`: Use the webhook secret from the production webhook you created in step 1.
4. `POSTGRES_URL`: Set this to your production database URL.
5. `AUTH_SECRET`: Set this to a random string. `openssl rand -base64 32` will generate one.
