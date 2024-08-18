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

class SapiV1CapitalConfigGetallGet200ResponseInnerNetworkListInner(BaseModel):
    """
    SapiV1CapitalConfigGetallGet200ResponseInnerNetworkListInner
    """ # noqa: E501
    address_regex: StrictStr = Field(alias="addressRegex")
    coin: StrictStr
    deposit_desc: StrictStr = Field(description="shown only when \"depositEnable\" is false.", alias="depositDesc")
    deposit_enable: StrictBool = Field(alias="depositEnable")
    is_default: StrictBool = Field(alias="isDefault")
    memo_regex: StrictStr = Field(alias="memoRegex")
    min_confirm: StrictInt = Field(description="min number for balance confirmation.", alias="minConfirm")
    name: StrictStr
    network: StrictStr
    special_tips: StrictStr = Field(alias="specialTips")
    un_lock_confirm: StrictInt = Field(description="confirmation number for balance unlock.", alias="unLockConfirm")
    withdraw_desc: StrictStr = Field(description="shown only when \"withdrawEnable\" is false", alias="withdrawDesc")
    withdraw_enable: StrictBool = Field(alias="withdrawEnable")
    withdraw_fee: StrictStr = Field(alias="withdrawFee")
    withdraw_integer_multiple: StrictStr = Field(alias="withdrawIntegerMultiple")
    withdraw_max: StrictStr = Field(alias="withdrawMax")
    withdraw_min: StrictStr = Field(alias="withdrawMin")
    same_address: StrictBool = Field(alias="sameAddress")
    __properties: ClassVar[List[str]] = ["addressRegex", "coin", "depositDesc", "depositEnable", "isDefault", "memoRegex", "minConfirm", "name", "network", "specialTips", "unLockConfirm", "withdrawDesc", "withdrawEnable", "withdrawFee", "withdrawIntegerMultiple", "withdrawMax", "withdrawMin", "sameAddress"]

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
        """Create an instance of SapiV1CapitalConfigGetallGet200ResponseInnerNetworkListInner from a JSON string"""
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
        """Create an instance of SapiV1CapitalConfigGetallGet200ResponseInnerNetworkListInner from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "addressRegex": obj.get("addressRegex"),
            "coin": obj.get("coin"),
            "depositDesc": obj.get("depositDesc"),
            "depositEnable": obj.get("depositEnable"),
            "isDefault": obj.get("isDefault"),
            "memoRegex": obj.get("memoRegex"),
            "minConfirm": obj.get("minConfirm"),
            "name": obj.get("name"),
            "network": obj.get("network"),
            "specialTips": obj.get("specialTips"),
            "unLockConfirm": obj.get("unLockConfirm"),
            "withdrawDesc": obj.get("withdrawDesc"),
            "withdrawEnable": obj.get("withdrawEnable"),
            "withdrawFee": obj.get("withdrawFee"),
            "withdrawIntegerMultiple": obj.get("withdrawIntegerMultiple"),
            "withdrawMax": obj.get("withdrawMax"),
            "withdrawMin": obj.get("withdrawMin"),
            "sameAddress": obj.get("sameAddress")
        })
        return _obj


