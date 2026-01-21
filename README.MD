# Overview

This repo is for a MCP server to talk to the Sudan Digital Archive API.

It is designed to be a stdio server shipped in a binary that runs on your own local machine.

## Gemini Set Up

Here is an example config you can add to a project's `.gemini/settings.json`:

```JSON
  "mcpServers": {
    "sudan-digital-archive": {
      "command": "path-to-binary",
      "args": [
        "--base-url",
        "https://api.sudandigitalarchive.com/sda-api",
        "--api-key",
        "YOUR_API_KEY"
      ]
    }
  }
```

## Testing

 - List subjects - success
 - Create subject - success
 - Delete subject - success
 - List accessions - needs test
 - List private accessions - needs test
 - Get one accession - success
 - Get one private accession - success
 - Update an accession - needs test

## To Dos

- Check the code; add linting and formatting
- Add docstrings
- Tidy up the readme
- Add some kind of build command to get it on path

## Known Issues

A lot of these errors:

│ x  list_subjects (sudan-digital-archive MCP Server) {"perPage":20,"page":2}                                                                                                                                      │
│                                                                                                                                                                                                                  │
│ no schema with key or ref "https://json-schema.org/draft/2020-12/schema"

But these are intermittent so seems related to gemini cli client to me...