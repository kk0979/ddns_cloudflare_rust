# ddns_cloudflare_rust

`ddns_cloudflare_rust` is a tool that implements Dynamic DNS (DDNS) by calling Cloudflare's DNS REST API to periodically update DNS records. The built executable, when run alongside a `.env` file, reads configurable parameters from the `.env` file to update DNS settings.

## Installation and Setup

1. Clone the repository:

   ```bash
   git clone https://github.com/kk0979/ddns_cloudflare_rust.git
   cd ./ddns_cloudflare_rust
Add a .env file in the project directory. The .env file should contain the following parameters:

   ```env
   API_TOKEN=your_api_token
   ZONE_ID=your_cloudflare_zone_id
   RECORD_ID=your_dns_a_record_id
   DOMAIN=your_domain
   TTL=your_ttl_in_seconds
   ```
Example .env file:

   ```dotenv
   API_TOKEN=_CwEvDvq5tpQqYF5BVxkGmGNjEn2JqSt4tVckJC3
   ZONE_ID=f32ebc346bb12c763da8e55bad29285b
   RECORD_ID=b924d1c417a0e83357523a0a5c48a985
   DOMAIN=site.com
   TTL=300
   ```
To build and run the application, use the following command:

   ```bash
   cargo run
   ```
To build release
   ```bash
   cargo build --release
   ```
You can find it on ./target/release

## Running Periodically (Optional)
If you want the program to run periodically, you can use crontab to schedule the execution.

Open the crontab editor:

   ```bash
   crontab -e
   ```
Add the following line to execute the program every 5 minutes:

   ```bash
   */5 * * * * /path/to/ddns_cloudflare_rust/target/release/ddns_cloudflare_rust
   ```
Replace /path/to/ddns_cloudflare_rust/target/release/ddns_cloudflare_rust with the actual path to your built executable.

To verify your cron jobs, use:

   ```bash
   crontab -l
   ```
