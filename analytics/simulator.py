"""Standalone transaction simulator utilities.
This module should provide helpers to call Solana RPC's simulateTransaction endpoint,
deserialize results, and compute expected slippage and price impact.
"""


def analyze_simulation(sim_result: dict) -> dict:
    """Convert raw RPC simulation result into normalized metrics."""
    # TODO: implement actual parsing
    return {
        'success': sim_result.get('ok', False),
        'compute_units_used': sim_result.get('compute_units', 0)
    }
