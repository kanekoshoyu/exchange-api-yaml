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

from pydantic import BaseModel, ConfigDict, Field, StrictBool, StrictInt, StrictStr
from typing import Any, ClassVar, Dict, List
from typing import Optional, Set
from typing_extensions import Self

class MyTrade(BaseModel):
    """
    MyTrade
    """ # noqa: E501
    symbol: StrictStr
    id: StrictInt = Field(description="Trade id")
    order_id: StrictInt = Field(alias="orderId")
    order_list_id: StrictInt = Field(alias="orderListId")
    price: StrictStr = Field(description="Price")
    qty: StrictStr = Field(description="Amount of base asset")
    quote_qty: StrictStr = Field(description="Amount of quote asset", alias="quoteQty")
    commission: StrictStr
    commission_asset: StrictStr = Field(alias="commissionAsset")
    time: StrictInt = Field(description="Trade timestamp")
    is_buyer: StrictBool = Field(alias="isBuyer")
    is_maker: StrictBool = Field(alias="isMaker")
    is_best_match: StrictBool = Field(alias="isBestMatch")
    __properties: ClassVar[List[str]] = ["symbol", "id", "orderId", "orderListId", "price", "qty", "quoteQty", "commission", "commissionAsset", "time", "isBuyer", "isMaker", "isBestMatch"]

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
        """Create an instance of MyTrade from a JSON string"""
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
        """Create an instance of MyTrade from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "symbol": obj.get("symbol"),
            "id": obj.get("id"),
            "orderId": obj.get("orderId"),
            "orderListId": obj.get("orderListId"),
            "price": obj.get("price"),
            "qty": obj.get("qty"),
            "quoteQty": obj.get("quoteQty"),
            "commission": obj.get("commission"),
            "commissionAsset": obj.get("commissionAsset"),
            "time": obj.get("time"),
            "isBuyer": obj.get("isBuyer"),
            "isMaker": obj.get("isMaker"),
            "isBestMatch": obj.get("isBestMatch")
        })
        return _obj


