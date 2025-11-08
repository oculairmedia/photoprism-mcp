# PhotoPrism MCP Tool Schemas - Examples

This document shows example JSON schemas that would be generated for the photo management tools. These schemas are what LLMs like Claude see when deciding which tools to use.

## Tool: search_photos

### Tool Definition (as seen by LLM)

```json
{
  "name": "search_photos",
  "description": "Search photos by text query. Use for basic searches without filters.",
  "inputSchema": {
    "type": "object",
    "properties": {
      "query": {
        "type": "string",
        "description": "Search query (can be empty to get all photos)"
      },
      "count": {
        "type": "integer",
        "description": "Maximum number of results (default: 100, max: 1000)",
        "minimum": 1,
        "maximum": 1000,
        "default": 100
      }
    },
    "required": ["query"]
  }
}
```

### Why This Works for LLMs

1. **Clear name**: `search_photos` directly matches user intent
2. **Simple description**: LLM knows this is for basic searches
3. **Minimal parameters**: Only query and optional count
4. **Constraints**: Min/max prevent invalid inputs
5. **Default**: LLM knows count defaults to 100

## Tool: search_photos_advanced

### Tool Definition (as seen by LLM)

```json
{
  "name": "search_photos_advanced",
  "description": "Search photos with advanced filters including type, quality, location, date, and more",
  "inputSchema": {
    "type": "object",
    "properties": {
      "q": {
        "type": "string",
        "description": "Text search query (searches titles, descriptions, keywords)"
      },
      "photo_type": {
        "type": "string",
        "enum": ["image", "video", "live", "raw"],
        "description": "Filter by photo type"
      },
      "quality": {
        "type": "integer",
        "minimum": 1,
        "maximum": 7,
        "description": "Filter by quality (1-7, where 7 is best)"
      },
      "favorite": {
        "type": "boolean",
        "description": "Only return favorite photos"
      },
      "country": {
        "type": "string",
        "description": "Filter by country name"
      },
      "state": {
        "type": "string",
        "description": "Filter by state/province"
      },
      "city": {
        "type": "string",
        "description": "Filter by city"
      },
      "year": {
        "type": "string",
        "description": "Filter by year taken (YYYY format, e.g., '2023')"
      },
      "month": {
        "type": "string",
        "description": "Filter by month taken (YYYY-MM format, e.g., '2023-06')"
      },
      "day": {
        "type": "string",
        "description": "Filter by specific day (YYYY-MM-DD format)"
      },
      "camera": {
        "type": "string",
        "description": "Filter by camera make/model"
      },
      "lens": {
        "type": "string",
        "description": "Filter by lens model"
      },
      "label": {
        "type": "string",
        "description": "Filter by label/category (e.g., 'sunset', 'portrait')"
      },
      "album": {
        "type": "string",
        "description": "Filter by album UID"
      },
      "subject": {
        "type": "string",
        "description": "Filter by person/subject UID"
      },
      "order": {
        "type": "string",
        "enum": ["newest", "oldest", "added", "edited", "name", "size", "relevance"],
        "description": "Sort order"
      },
      "count": {
        "type": "integer",
        "minimum": 1,
        "maximum": 1000,
        "default": 100,
        "description": "Maximum number of results to return"
      },
      "offset": {
        "type": "integer",
        "minimum": 0,
        "default": 0,
        "description": "Pagination offset"
      }
    },
    "required": []
  }
}
```

### Why This Works for LLMs

1. **Comprehensive options**: LLM can see all available filters
2. **Enums**: Constrained choices prevent invalid values
3. **No required fields**: LLM can use any combination of filters
4. **Clear descriptions**: Each filter's purpose is obvious
5. **Format hints**: Date formats explained ("YYYY-MM-DD")
6. **Examples in descriptions**: Helps LLM understand usage

## Tool: update_photo

### Tool Definition (as seen by LLM)

```json
{
  "name": "update_photo",
  "description": "Update photo metadata and attributes",
  "inputSchema": {
    "type": "object",
    "properties": {
      "uid": {
        "type": "string",
        "description": "Photo unique identifier (16 characters, required)",
        "minLength": 16,
        "maxLength": 16
      },
      "title": {
        "type": "string",
        "description": "New title for the photo"
      },
      "description": {
        "type": "string",
        "description": "New description/caption"
      },
      "favorite": {
        "type": "boolean",
        "description": "Set favorite status"
      },
      "private": {
        "type": "boolean",
        "description": "Set private/hidden status"
      },
      "quality": {
        "type": "integer",
        "minimum": 1,
        "maximum": 7,
        "description": "Override photo quality (1-7)"
      },
      "keywords": {
        "type": "array",
        "items": {
          "type": "string"
        },
        "description": "Update keywords/tags"
      },
      "lat": {
        "type": "number",
        "description": "Update latitude"
      },
      "lng": {
        "type": "number",
        "description": "Update longitude"
      },
      "altitude": {
        "type": "integer",
        "description": "Update altitude in meters"
      },
      "country": {
        "type": "string",
        "description": "Update country"
      },
      "place": {
        "type": "string",
        "description": "Update place name"
      }
    },
    "required": ["uid"]
  }
}
```

### Why This Works for LLMs

1. **Only UID required**: LLM knows it must get UID from previous search
2. **All fields optional**: Supports partial updates naturally
3. **Length validation**: Prevents invalid UIDs
4. **Type variety**: Demonstrates different data types (bool, string, number, array)
5. **Clear naming**: `favorite` not `isFavorite`, matches natural language

## Tool: delete_photo

### Tool Definition (as seen by LLM)

```json
{
  "name": "delete_photo",
  "description": "Delete a photo permanently from PhotoPrism. WARNING: This operation is irreversible.",
  "inputSchema": {
    "type": "object",
    "properties": {
      "uid": {
        "type": "string",
        "description": "Photo unique identifier (16 characters, required)",
        "minLength": 16,
        "maxLength": 16
      }
    },
    "required": ["uid"]
  }
}
```

### Why This Works for LLMs

1. **WARNING in description**: LLM sees this is destructive
2. **Simple interface**: Just UID, reduces chance of errors
3. **Clear validation**: 16-character constraint explicit
4. **Required field**: LLM knows UID is mandatory

## Conversation Examples with Schema Usage

### Example 1: Smart Tool Selection

**User**: "Find my photos from last summer at the beach"

**LLM Reasoning**:
1. Multiple criteria mentioned (location + time)
2. Checks available tools
3. Sees `search_photos_advanced` has `q`, `month`, and other filters
4. Selects `search_photos_advanced` over simple `search_photos`

**Tool Call**:
```json
{
  "tool": "search_photos_advanced",
  "parameters": {
    "q": "beach",
    "month": "2024-06",
    "count": 100
  }
}
```

### Example 2: Enum Understanding

**User**: "Show me only videos"

**LLM Reasoning**:
1. User wants to filter by type
2. Checks `photo_type` parameter schema
3. Sees enum: `["image", "video", "live", "raw"]`
4. Selects "video" from enum

**Tool Call**:
```json
{
  "tool": "search_photos_advanced",
  "parameters": {
    "photo_type": "video"
  }
}
```

### Example 3: Validation Awareness

**User**: "Get details of photo abc123"

**LLM Reasoning**:
1. User provided UID "abc123"
2. Checks `get_photo` schema
3. Sees UID must be 16 characters
4. Recognizes this is incomplete
5. Asks user for complete UID

**LLM Response**: "I need the complete 16-character UID to retrieve the photo details. The UID you provided ('abc123') is only 6 characters. Could you provide the full UID?"

### Example 4: Default Value Usage

**User**: "Search for sunset photos"

**LLM Reasoning**:
1. Simple search, uses `search_photos`
2. Sees `count` has default value 100
3. Omits `count` parameter, relying on default

**Tool Call**:
```json
{
  "tool": "search_photos",
  "parameters": {
    "query": "sunset"
  }
}
```

### Example 5: Batch Operation

**User**: "Mark all these photos as favorites" (after a search)

**LLM Reasoning**:
1. Has array of UIDs from previous search
2. Checks for batch operations
3. Finds `like_photos` tool with `uids` array parameter
4. Extracts UIDs from search results
5. Calls batch tool

**Tool Call**:
```json
{
  "tool": "like_photos",
  "parameters": {
    "uids": ["abc123def4567890", "xyz789abc1234567", ...]
  }
}
```

## Schema Design Principles Demonstrated

### 1. Enums for Constrained Choices

```json
{
  "photo_type": {
    "enum": ["image", "video", "live", "raw"]
  }
}
```

**Benefit**: LLM can't pass invalid values, sees all options

### 2. Range Constraints

```json
{
  "quality": {
    "minimum": 1,
    "maximum": 7
  }
}
```

**Benefit**: Clear boundaries, LLM won't try quality=10

### 3. Format Hints in Descriptions

```json
{
  "year": {
    "description": "Filter by year taken (YYYY format, e.g., '2023')"
  }
}
```

**Benefit**: LLM knows exact format expected

### 4. Defaults Reduce Verbosity

```json
{
  "count": {
    "default": 100
  }
}
```

**Benefit**: LLM doesn't need to always specify, reduces token usage

### 5. Optional Parameters = Flexibility

```json
{
  "required": ["uid"]  // Only UID required
}
```

**Benefit**: LLM can provide any subset of updates

## Token Efficiency

### Compact Schema Example

Instead of this verbose API:
```json
{
  "search": {
    "filters": {
      "text": {"value": "sunset"},
      "type": {"value": "image"},
      "metadata": {
        "favorite": {"enabled": true}
      }
    },
    "pagination": {
      "limit": 100,
      "offset": 0
    }
  }
}
```

We use this flat schema:
```json
{
  "q": "sunset",
  "photo_type": "image",
  "favorite": true,
  "count": 100
}
```

**Token Savings**: ~60% fewer tokens for same information

## Schema Validation Benefits

### 1. Type Safety

```json
{
  "favorite": {
    "type": "boolean"
  }
}
```

LLM won't send `"favorite": "yes"` or `"favorite": "true"` (string)

### 2. Array Type Specification

```json
{
  "keywords": {
    "type": "array",
    "items": {"type": "string"}
  }
}
```

LLM knows to send `["tag1", "tag2"]` not `"tag1, tag2"`

### 3. Number vs Integer

```json
{
  "lat": {"type": "number"},     // 37.7749
  "altitude": {"type": "integer"} // 100
}
```

LLM sends appropriate precision

## Conclusion

These schemas demonstrate how proper type design and documentation create an optimal interface for LLM interaction. Key takeaways:

1. **Clear names** match user language
2. **Enums** prevent invalid inputs
3. **Descriptions** guide usage
4. **Defaults** reduce verbosity
5. **Validation** catches errors early
6. **Examples** clarify intent
7. **Flat structures** save tokens
8. **Optional fields** increase flexibility

The schemas are automatically generated from Rust types using `schemars`, ensuring they stay in sync with the implementation while maintaining LLM usability.
