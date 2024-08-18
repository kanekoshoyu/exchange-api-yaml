# coding: utf-8

"""
    Binance Public Spot API

    OpenAPI Specifications for the Binance Public Spot API  API documents:   - [https://github.com/binance/binance-spot-api-docs](https://github.com/binance/binance-spot-api-docs)   - [https://binance-docs.github.io/apidocs/spot/en](https://binance-docs.github.io/apidocs/spot/en)

    The version of the OpenAPI document: 1.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


from __future__ import annotations
import pprint
import re  # noqa: F401
import json

from pydantic import BaseModel, ConfigDict, Field, StrictInt, StrictStr
from typing import Any, ClassVar, Dict, List
from openapi_client.models.api_v3_exchange_info_get200_response_rate_limits_inner import ApiV3ExchangeInfoGet200ResponseRateLimitsInner
from openapi_client.models.api_v3_exchange_info_get200_response_symbols_inner import ApiV3ExchangeInfoGet200ResponseSymbolsInner
from typing import Optional, Set
from typing_extensions import Self

class ApiV3ExchangeInfoGet200Response(BaseModel):
    """
    ApiV3ExchangeInfoGet200Response
    """ # noqa: E501
    timezone: StrictStr
    server_time: StrictInt = Field(alias="serverTime")
    rate_limits: List[ApiV3ExchangeInfoGet200ResponseRateLimitsInner] = Field(alias="rateLimits")
    exchange_filters: List[Dict[str, Any]] = Field(alias="exchangeFilters")
    symbols: List[ApiV3ExchangeInfoGet200ResponseSymbolsInner]
    __properties: ClassVar[List[str]] = ["timezone", "serverTime", "rateLimits", "exchangeFilters", "symbols"]

    model_config = ConfigDict(
        populate_by_name=True,
        validate_assignment=True,
        protected_namespaces=(),
    )


    def to_str(self) -> str:
        """Returns the string representation of the model using alias"""
        return pprint.pformat(self.model_dump(by_alias=True))

    def to_json(self) -> str:
        """Returns the JSON representation of the model using alias"""
        # TODO: pydantic v2: use .model_dump_json(by_alias=True, exclude_unset=True) instead
        return json.dumps(self.to_dict())

    @classmethod
    def from_json(cls, json_str: str) -> Optional[Self]:
        """Create an instance of ApiV3ExchangeInfoGet200Response from a JSON string"""
        return cls.from_dict(json.loads(json_str))

    def to_dict(self) -> Dict[str, Any]:
        """Return the dictionary representation of the model using alias.

        This has the following differences from calling pydantic's
        `self.model_dump(by_alias=True)`:

        * `None` is only added to the output dict for nullable fields that
          were set at model initialization. Other fields with value `None`
          are ignored.
        """
        excluded_fields: Set[str] = set([
        ])

        _dict = self.model_dump(
            by_alias=True,
            exclude=excluded_fields,
            exclude_none=True,
        )
        # override the default output from pydantic by calling `to_dict()` of each item in rate_limits (list)
        _items = []
        if self.rate_limits:
            for _item in self.rate_limits:
                if _item:
                    _items.append(_item.to_dict())
            _dict['rateLimits'] = _items
        # override the default output from pydantic by calling `to_dict()` of each item in symbols (list)
        _items = []
        if self.symbols:
            for _item in self.symbols:
                if _item:
                    _items.append(_item.to_dict())
            _dict['symbols'] = _items
        return _dict

    @classmethod
    def from_dict(cls, obj: Optional[Dict[str, Any]]) -> Optional[Self]:
        """Create an instance of ApiV3ExchangeInfoGet200Response from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "timezone": obj.get("timezone"),
            "serverTime": obj.get("serverTime"),
            "rateLimits": [ApiV3ExchangeInfoGet200ResponseRateLimitsInner.from_dict(_item) for _item in obj["rateLimits"]] if obj.get("rateLimits") is not None else None,
            "exchangeFilters": obj.get("exchangeFilters"),
            "symbols": [ApiV3ExchangeInfoGet200ResponseSymbolsInner.from_dict(_item) for _item in obj["symbols"]] if obj.get("symbols") is not None else None
        })
        return _obj


