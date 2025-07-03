import { DatapointProperties } from '../../generated/DatapointProperties';

export function getDatapointStatus(
  name: string,
  value: number,
  timestamp: number
) {
  const props = (DatapointProperties as Record<string, { lower: number | null; upper: number | null; stale_after: number | null; critical: boolean | null }>)[name];
  const now = Date.now() / 1000; // seconds

  const isOutOfRange =
    props &&
    ((props.lower !== null && value < props.lower) ||
     (props.upper !== null && value > props.upper));

  const isStale =
    props &&
    props.stale_after !== null &&
    (now - timestamp > props.stale_after);

  const isCritical = props && props.critical === true;
  const isEmergency = isCritical && (isOutOfRange || isStale);

  return { isOutOfRange, isStale, isCritical, isEmergency };
} 