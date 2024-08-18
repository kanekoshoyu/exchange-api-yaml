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
from typing import Optional, Set
from typing_extensions import Self

class ApiV3TickerGet200Response(BaseModel):
    """
    ApiV3TickerGet200Response
    """ # noqa: E501
    symbol: StrictStr
    price_change: StrictStr = Field(alias="priceChange")
    price_change_percent: StrictStr = Field(alias="priceChangePercent")
    weighted_avg_price: StrictStr = Field(alias="weightedAvgPrice")
    open_price: StrictStr = Field(alias="openPrice")
    high_price: StrictStr = Field(alias="highPrice")
    low_price: StrictStr = Field(alias="lowPrice")
    last_price: StrictStr = Field(alias="lastPrice")
    volume: StrictStr
    quote_volume: StrictStr = Field(alias="quoteVolume")
    open_time: StrictInt = Field(alias="openTime")
    close_time: StrictInt = Field(alias="closeTime")
    first_id: StrictInt = Field(alias="firstId")
    last_id: StrictInt = Field(alias="lastId")
    count: StrictInt
    __properties: ClassVar[List[str]] = ["symbol", "priceChange", "priceChangePercent", "weightedAvgPrice", "openPrice", "highPrice", "lowPrice", "lastPrice", "volume", "quoteVolume", "openTime", "closeTime", "firstId", "lastId", "count"]

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
        """Create an instance of ApiV3TickerGet200Response from a JSON string"""
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
        return _dict

    @classmethod
    def from_dict(cls, obj: Optional[Dict[str, Any]]) -> Optional[Self]:
        """Create an instance of ApiV3TickerGet200Response from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "symbol": obj.get("symbol"),
            "priceChange": obj.get("priceChange"),
            "priceChangePercent": obj.get("priceChangePercent"),
            "weightedAvgPrice": obj.get("weightedAvgPrice"),
            "openPrice": obj.get("openPrice"),
            "highPrice": obj.get("highPrice"),
            "lowPrice": obj.get("lowPrice"),
            "lastPrice": obj.get("lastPrice"),
            "volume": obj.get("volume"),
            "quoteVolume": obj.get("quoteVolume"),
            "openTime": obj.get("openTime"),
            "closeTime": obj.get("closeTime"),
            "firstId": obj.get("firstId"),
            "lastId": obj.get("lastId"),
            "count": obj.get("count")
        })
        return _obj


