# rembirth

A command-line tool to remember and track birthdays.

## Installation

```bash
cargo build --release
cp target/release/rembirth ~/.local/bin/rembirth
```

## Commands

### rembirth new

Add a new birthday to the database.

```bash
rembirth new <name> <month> <day> <year>
```

- `name` - Person's name (string)
- `month` - Month of birth (1-12)
- `day` - Day of birth (1-31)
- `year` - Year of birth (e.g., 1990)

**Example:**
```bash
rembirth new Alice 6 15 1995
```

### rembirth get

List all birthdays in the database in a formatted table.

```bash
rembirth get
```

**Example output:**
```
+----+-------+-------+------+--------+
| id | name  | month | day  | year   |
+----+-------+-------+------+--------+
| 1  | Alice | 6     | 15   | 1995   |
| 2  | Bob   | 3     | 22   | 1990   |
+----+-------+-------+------+--------+
```

### rembirth next

Show the next upcoming birthday, sorted by days remaining

```bash
rembirth next
```

**Example output:**

```
Closest birthday ID: Some(1), in 45 days
```

### rembirth --help

Display help commands

```bash
rembirth --help
```

## Database

Birthdays are stored in `birth.db` in the main dierectory. The database is created automatically. 

