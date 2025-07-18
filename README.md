# Podcast AI

An intelligent podcast generation system that fetches original tweets from X (Twitter) and transforms them into AI-generated podcast content. This system processes social media content to create engaging audio experiences.

## 🎯 Description

Podcast AI is a Rust-based backend service that:

- **Fetches original tweets** from any public X (Twitter) user
- **Filters content** to exclude replies, retweets, and quote tweets  
- **Processes text** for AI consumption by removing URLs and noise
- **Provides clean API endpoints** for podcast generation workflows
- **Optimizes for AI models** with structured, context-ready text output

The system serves as the data pipeline for AI-powered podcast creation, transforming social media content into podcast-ready material.

## 🚀 Quick Start

### Prerequisites

- **Rust 1.70+** installed ([Install Rust](https://rustup.rs/))
- **Twitter Developer Account** with Bearer Token
- **Git** for cloning the repository

### Installation

1. **Clone the repository**:
```bash
git clone <your-repository-url>
cd amplify
```

2. **Navigate to backend directory**:
```bash
cd backend
```

3. **Set up environment variables**:
```bash
# Create .env file with your Twitter Bearer Token
echo "BEARER_TOKEN=your_actual_twitter_bearer_token_here" > .env
```

> ⚠️ **Important**: Replace `your_actual_twitter_bearer_token_here` with your actual Twitter API Bearer Token

4. **Install dependencies and build**:
```bash
cargo build
```

5. **Run the server**:
```bash
cargo run
```

The server will start on `http://127.0.0.1:8080` and display:
```
🖥️  Server running on port 8080
```

## 🛠️ Tech Stack

### Backend
- **Rust** - Systems programming language for performance and safety
- **Actix Web** - High-performance web framework for Rust
- **Tokio** - Asynchronous runtime for Rust
- **Reqwest** - HTTP client for API requests
- **Serde** - Serialization/deserialization framework
- **Regex** - Text processing and URL cleaning
- **dotenvy** - Environment variable management

### External APIs
- **X (Twitter) API v2** - Tweet data retrieval
- **Bearer Token Authentication** - Secure API access

### Data Format
- **JSON** - API response format
- **Clean Text** - Processed output for AI consumption

## 📡 API Endpoints

### 1. Get Original Tweets (Raw Data)
```http
GET /tweets/original
```

**Description**: Fetches original tweets with full metadata and public metrics.

**Query Parameters**:
- `username` (required): Twitter username without @ symbol
- `max` (optional): Number of tweets (10-100, default: 20)

**Example Request**:
```bash
curl "http://127.0.0.1:8080/tweets/original?username=Rustix69&max=10"
```

**Example Response**:
```json
[
  {
    "id": "1945690992981717364",
    "edit_history_tweet_ids": ["1945690992981717364"],
    "created_at": "2025-07-17T03:44:16.000Z",
    "text": "People who choose themselves always win no matter how bad the situation gets.",
    "public_metrics": {
      "retweet_count": 0,
      "reply_count": 0,
      "like_count": 5,
      "quote_count": 0,
      "bookmark_count": 1,
      "impression_count": 224
    }
  }
]
```

### 2. Get Processed Tweets (AI-Ready)
```http
GET /tweets/processed
```

**Description**: Fetches and processes tweets into clean text format optimized for AI/Context APIs.

**Query Parameters**:
- `username` (required): Twitter username without @ symbol  
- `max` (optional): Number of tweets (10-100, default: 20)

**Example Request**:
```bash
curl "http://127.0.0.1:8080/tweets/processed?username=Rustix69&max=10"
```

**Example Response**:
```json
{
  "username": "Rustix69",
  "tweet_count": 10,
  "processed_text": "Here are the recent tweets from @Rustix69 to be made into a podcast:\n\nPeople who choose themselves always win no matter how bad the situation gets.\n\nWaiting for the NYC !!!\n\nLFG 🚀 Hope so Gold will respect my levels. Otherwise C gaye guru.\n\nWent from mom's little boy to her biggest disappointment. Will be turning 21 next month but it feels like nothing great has happened."
}
```

## 🔧 Environment Configuration

Create a `.env` file in the `backend/` directory:

```env
# Twitter API Bearer Token (Required)
BEARER_TOKEN=your_twitter_bearer_token_here
```

### Getting a Twitter Bearer Token

1. Go to [Twitter Developer Portal](https://developer.twitter.com/)
2. Create a new app or use existing app
3. Navigate to "Keys and Tokens"
4. Generate/copy your "Bearer Token"
5. Add it to your `.env` file

## 🧪 Testing

### Run Unit Tests
```bash
cargo test
```

### Test with cURL
```bash
# Test processed endpoint (recommended for AI)
curl "http://127.0.0.1:8080/tweets/processed?username=elonmusk&max=5"

# Test original endpoint (full data)
curl "http://127.0.0.1:8080/tweets/original?username=elonmusk&max=5"
```

### Test with Postman
1. **Method**: GET
2. **URL**: `http://127.0.0.1:8080/tweets/processed`
3. **Params**: 
   - `username`: `elonmusk`
   - `max`: `10`

## 📁 Project Structure

```
backend/
├── src/
│   ├── main.rs                    # Application entry point
│   └── api/
│       ├── mod.rs                 # API module declarations
│       ├── routes.rs              # Route configuration
│       ├── controllers/
│       │   ├── mod.rs
│       │   └── tweet_controller.rs # Tweet endpoint handlers
│       ├── services/
│       │   ├── mod.rs
│       │   └── tweet_service.rs    # Twitter API integration
│       └── models/
│           ├── mod.rs
│           └── tweet.rs           # Data models
├── Cargo.toml                     # Dependencies
├── Cargo.lock                     # Dependency lockfile
└── .env                          # Environment variables
```

## 🔄 Development Workflow

### For Podcast Generation:
1. **Fetch tweets**: Call `/tweets/processed` endpoint
2. **Extract text**: Get the `processed_text` field
3. **Send to AI**: Use the clean text with your Context API
4. **Generate podcast**: Process with your AI model

### Data Flow:
```
Twitter API → Rust Backend → Clean Text → AI/Context API → Podcast
```

## ⚡ Performance & Limits

- **Rate Limits**: Twitter API allows 300 requests per 15-minute window
- **Max Tweets**: 10-100 tweets per request
- **Response Time**: Typically <2 seconds for 20 tweets
- **Concurrent Requests**: Supported via Actix Web async handling

## 🛡️ Error Handling

The API returns appropriate HTTP status codes:

- **200 OK**: Successful request
- **400 Bad Request**: Invalid parameters or Twitter API errors
- **500 Internal Server Error**: Server errors

**Error Response Format**:
```json
{
  "error": "No tweets found"
}
```

## 📄 License

This project is licensed under the MIT License.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable  
5. Submit a pull request

---

**Built with ❤️ using Rust and Actix Web** 