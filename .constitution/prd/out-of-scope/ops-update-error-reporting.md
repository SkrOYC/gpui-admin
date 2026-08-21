# Out of Scope: Framework Update Checks & Error Reporting Helpers

## Context
Built-in application-update notification or error-export glue shipped by the framework.

## Decision
`rejected`

## Reason
gpui-admin is a library, not a product: distribution, update checking, and error reporting are wholly the Adopter application's concerns. The structured-logging subscriber seam already lets Adopters attach any exporter; shipping helpers duplicates adopter-owned surface and erodes the no-phone-home posture (NFC-31).

## Consequences
None anticipated; reopening would contradict the governing library principle.
